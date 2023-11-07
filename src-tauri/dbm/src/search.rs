use std::{path::PathBuf, time::Duration};

use chrono::Utc;
use debounced::{debounced_if, Debounced};
use futures_channel::mpsc;
use futures_util::SinkExt;
use sea_orm::{DatabaseConnection, EntityTrait};
use tantivy::{
  directory::MmapDirectory,
  doc,
  query::QueryParser,
  schema::{Schema, STORED, TEXT},
  Index,
};
use tokio::sync::oneshot;
use tokio_stream::StreamExt;
use tracing::{error, trace};

use crate::{MusyncError, TrackId};

struct SearchActor {
  receiver: Debounced<mpsc::Receiver<ActorMessage>>,
  db: DatabaseConnection,
  index: Index,
}

enum ActorMessage {
  Index,
  Search {
    query: String,
    respond_to: oneshot::Sender<SearchResult>,
  },
}

impl SearchActor {
  fn new(
    receiver: Debounced<mpsc::Receiver<ActorMessage>>,
    db: DatabaseConnection,
    cache_data_folder: PathBuf,
  ) -> Self {
    Self {
      receiver,
      db,
      index: Self::init(cache_data_folder),
    }
  }

  fn init(cache_data_folder: PathBuf) -> Index {
    let mut schema_builder = Schema::builder();
    schema_builder.add_i64_field("id", STORED);
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("artist", TEXT | STORED);
    schema_builder.add_text_field("album", TEXT | STORED);
    let schema = schema_builder.build();
    if !cache_data_folder.exists() {
      std::fs::create_dir_all(&cache_data_folder).expect("create cache data folder failed");
    }
    let index = Index::open_or_create(
      MmapDirectory::open(cache_data_folder).expect("open cache data folder failed"),
      schema,
    )
    .expect("create index failed");
    let tokenizer = tantivy_jieba::JiebaTokenizer {};
    index.tokenizers().register("jieba", tokenizer);
    index
  }

  async fn rebuild_index(&mut self) {
    let start = Utc::now();
    // 100MB
    let mut index_writer = self
      .index
      .writer(100_000_000)
      .expect("create writer failed");
    index_writer
      .delete_all_documents()
      .expect("delete all documents failed");
    index_writer.commit().expect("commit failed");
    let tracks = match entity::track::Entity::find().all(&self.db).await {
      Err(e) => {
        error!("find all tracks failed: {:?}", e);
        return;
      }
      Ok(track) => track,
    };
    let id = self.index.schema().get_field("id").unwrap();
    let title = self.index.schema().get_field("title").unwrap();
    let artist = self.index.schema().get_field("artist").unwrap();
    let album = self.index.schema().get_field("album").unwrap();

    for track in tracks {
      let doc = doc!(
        id => track.id as i64,
        title => track.title,
        artist => track.artist.unwrap_or_default(),
        album => track.album.unwrap_or_default(),
      );
      index_writer
        .add_document(doc)
        .expect("add doc should not fail");
    }
    index_writer.commit().expect("commit failed");
    let end = Utc::now();
    trace!("index duration: {:?}", end - start);
  }

  async fn handle_message(&mut self, msg: ActorMessage) {
    match msg {
      ActorMessage::Index => {
        trace!("indexing...");
        self.rebuild_index().await;
      }
      ActorMessage::Search { query, respond_to } => {
        trace!("searching: {query}");
        let start = Utc::now();
        let reader = self
          .index
          .reader_builder()
          .reload_policy(tantivy::ReloadPolicy::OnCommit)
          .try_into()
          .unwrap();
        let searcher = reader.searcher();
        let id = self.index.schema().get_field("id").unwrap();
        let title = self.index.schema().get_field("title").unwrap();
        let artist = self.index.schema().get_field("artist").unwrap();
        let album = self.index.schema().get_field("album").unwrap();
        let query_parser = QueryParser::for_index(&self.index, vec![title, artist, album]);
        let query = query_parser.parse_query(&query).unwrap();
        let top_docs = searcher
          .search(&query, &tantivy::collector::TopDocs::with_limit(20))
          .unwrap();
        let track_ids = top_docs
          .iter()
          .map(|(_, doc)| {
            let retrieved_doc = searcher.doc(*doc).unwrap();
            let id = retrieved_doc.get_first(id).unwrap().as_i64().unwrap();
            id as i32
          })
          .collect::<Vec<_>>();
        for (score, doc_address) in top_docs {
          let retrieved_doc = searcher.doc(doc_address).unwrap();
          trace!(
            "score: {score}, doc: {}",
            self.index.schema().to_json(&retrieved_doc)
          );
        }
        respond_to.send(track_ids).unwrap();
        let end = Utc::now();
        trace!("search duration: {:?}", end - start);
      }
    }
  }
}

async fn run_search_actor(mut actor: SearchActor) {
  while let Some(msg) = actor.receiver.next().await {
    actor.handle_message(msg).await;
  }
}

#[derive(Clone, Debug)]
pub struct SearchActorHandle {
  sender: mpsc::Sender<ActorMessage>,
}

pub type SearchResult = Vec<TrackId>;

impl SearchActorHandle {
  pub fn new(db: DatabaseConnection, data_folder: PathBuf) -> Self {
    let (sender, receiver) = mpsc::channel(32);
    let debounced = debounced_if(receiver, Duration::from_secs(20), |msg| {
      !matches!(msg, ActorMessage::Index)
    });
    let actor = SearchActor::new(debounced, db, data_folder.join("tantivy"));
    tokio::spawn(run_search_actor(actor));
    Self { sender }
  }

  pub async fn index(&self) {
    let _msg = ActorMessage::Index;
    // let mut sender = self.sender.clone();
    // sender.send(msg).await.with_err(|e| {
    //   error!("error sending message to search actor: {:?}", e);
    // });
    // trace!("index message sent")
  }

  pub async fn search(&self, query: String) -> Result<SearchResult, MusyncError> {
    let (send, recv) = oneshot::channel();
    let msg = ActorMessage::Search {
      query,
      respond_to: send,
    };
    let mut sender: mpsc::Sender<ActorMessage> = self.sender.clone();
    sender.send(msg).await.map_err(|e| {
      error!("error sending message to search actor: {:?}", e);
      crate::error::MusyncError::SearchActor
    })?;
    trace!("search message sent");
    recv.await.map_err(|e| {
      error!("error receiving search results: {:?}", e);
      crate::error::MusyncError::SearchActor
    })
  }
}
