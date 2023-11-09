export interface Theme {
  main: string
  highlight: {
    black: string
    white: string
  }
}

export const defaultTheme: Theme = {
  main: '#4cc2ff',
  highlight: { black: '#4b4b4b60', white: '#fdfefe' },
};
