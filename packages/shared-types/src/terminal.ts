export interface TerminalResizeMessage {
  type: 'resize';
  cols: number;
  rows: number;
}

export interface TerminalDataMessage {
  type: 'data';
  data: string;
}

export interface TerminalPingMessage {
  type: 'ping';
}

export type TerminalClientMessage =
  | TerminalResizeMessage
  | TerminalDataMessage
  | TerminalPingMessage;

export interface TerminalSessionInfo {
  sessionId: string;
  containerId: string;
  user: string;
  activeSince: string;
}
