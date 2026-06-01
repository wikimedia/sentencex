export interface SentenceBoundary {
  /** Character index where the sentence starts. */
  start_index: number;
  /** Character index where the sentence ends. */
  end_index: number;
  /** The sentence text. */
  text: string;
  /** Punctuation mark that ended the sentence, or null if none. */
  boundary_symbol: string | null;
  /** Whether this boundary represents a paragraph break ("\n\n"). */
  is_paragraph_break: boolean;
}

export function segment(language: string, text: string): string[];
export function get_sentence_boundaries(
  language: string,
  text: string,
): SentenceBoundary[];

declare const _default: typeof segment;
export default _default;
