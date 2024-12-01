/* tslint:disable */
/* eslint-disable */
export function greet(): void;
/**
 * @returns {string}
 */
export function version(): string;
/**
 * @param {number} size
 * @returns {Board}
 */
export function init_othello(size: number): Board;
/**
 * @param {Board} board
 * @param {Piece} player
 * @returns {number}
 */
export function placeable(board: Board, player: Piece): number;
/**
 * @param {Board} board
 * @param {Point} at
 * @param {Piece} player
 * @returns {boolean}
 */
export function can_place(board: Board, at: Point, player: Piece): boolean;
/**
 * @param {Board} board
 * @returns {Scores}
 */
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
 * @param {Board} board
 * @param {Piece} player
 * @param {Point} at
 * @returns {Board}
 */
export function place_at(board: Board, player: Piece, at: Point): Board;
/**
 * @param {Board} board
 * @param {Piece} ai_player
 * @param {number} strength
 * @returns {Board}
 */
export function generate_ai_play(board: Board, ai_player: Piece, strength: number): Board;
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
  free(): void;
  /**
   * is an (relatively) expensive operation, so better cached than done every access
   * should return Array<Array<"." | "b" | "w">>
   * @returns {any}
   */
  get_data(): any;
  size: number;
}
export class PlaceError {
  free(): void;
}
export class Point {
  free(): void;
  /**
   * @param {number} x
   * @param {number} y
   * @returns {Point}
   */
  static create(x: number, y: number): Point;
  x: number;
  y: number;
}
export class Scores {
  free(): void;
  black: number;
  white: number;
}
