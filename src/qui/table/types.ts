export interface QTableColumn {
  key: string
  title?: string
  style?: {
    gridTemplateColumn?: string
    textAlign?: 'left' | 'center' | 'right'
    hidePx?: number
  }
}
