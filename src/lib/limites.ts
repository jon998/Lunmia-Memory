/** Tope de campos de texto. En caracteres (Unicode), no en palabras:
 *  - 4 000 líneas ≈ 200 000 caracteres; estos cortes lo impiden.
 *  - Una palabra en español ≈ 6–7 caracteres; el comentario indica el equivalente. */

export const LIMITES = {
  /** Home + flotante. ~1 200 palabras / ~150 líneas. Cabe un mail o un stack; no un dump. */
  captura: 8_000,
  /** Editor de entrada. Un poco más de aire tras capturar. */
  entrada: 12_000,
  /** Onboarding «¿a qué te dedicas?». ~250 palabras, 4–5 párrafos de café. */
  oficio: 1_500,
  /** Espacios, tipos, proyectos, etiquetas. */
  nombre: 40,
  /** Corrección rápida del panel post-captura. */
  prompt: 500,
  /** Buscador del home. */
  busqueda: 200
} as const;

export function cercaDelLimite(valor: string, max: number, umbral = 0.8): boolean {
  return valor.length >= Math.floor(max * umbral);
}
