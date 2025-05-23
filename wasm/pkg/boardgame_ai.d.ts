/* tslint:disable */
/* eslint-disable */
export function greet(): void;
export function version(): string;
export function init_othello(size: number): Board;
export function placeable(board: Board, player: Piece): number;
export function can_place(board: Board, at: Point, player: Piece): boolean;
export function count_score(board: Board): Scores;
/**
 * return either Ok(encoded board) or Err(error)
 * encoding specs:
 *
 * - board must be encoded correctly
 * - player must be "w" or "b"
 * - at must be encoded as "{x},{y}"
 *
 * fails if:
 *
 * - the point is already occupied
 * - the point is not placeable place
 */
export function place_at(board: Board, player: Piece, at: Point): Board;
export function generate_ai_play(board: Board, ai_player: Piece, strength: number): Board;
export function score(board: Board, player: Piece): number;
export enum Cell {
  Empty = 0,
  Black = 1,
  White = 2,
}
export enum Piece {
  Black = 0,
  White = 1,
}
export class Board {
  private constructor();
  free(): void;
  /**
   * is an (relatively) expensive operation, so better cached than done every access
   * should return Array<Array<"." | "b" | "w">>
   */
  get_data(): any;
  size(): number;
  score(player: Piece): number;
  size: number;
}
export class PlaceError {
  private constructor();
  free(): void;
}
export class Point {
  private constructor();
  free(): void;
  static create(x: number, y: number): Point;
  x: number;
  y: number;
}
export class Scores {
  private constructor();
  free(): void;
  black: number;
  white: number;
}
