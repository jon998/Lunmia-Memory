export type Bloque =
  | { kind: 'texto'; valor: string }
  | { kind: 'codigo'; lang: string; valor: string };

const VALLA = /```([\w+-]*)\r?\n?([\s\S]*?)```/g;

export function parsearContenido(src: string): Bloque[] {
  const bloques: Bloque[] = [];
  let ultimo = 0;
  const re = new RegExp(VALLA.source, 'g');
  let m: RegExpExecArray | null;
  while ((m = re.exec(src)) !== null) {
    const antes = src.slice(ultimo, m.index);
    if (antes.trim()) bloques.push({ kind: 'texto', valor: antes.trim() });
    bloques.push({
      kind: 'codigo',
      lang: (m[1] || 'txt').toLowerCase(),
      valor: m[2].replace(/\n$/, '')
    });
    ultimo = m.index + m[0].length;
  }
  const resto = src.slice(ultimo);
  if (resto.trim()) bloques.push({ kind: 'texto', valor: resto.trim() });
  if (bloques.length === 0 && src) bloques.push({ kind: 'texto', valor: src });
  return bloques;
}

export function tieneCodigo(src: string): boolean {
  return parsearContenido(src).some((b) => b.kind === 'codigo');
}

/** Primera línea usable para listas: sin vallas ni backticks. */
export function previewLista(src: string): string {
  const limpio = src
    .replace(/```[\w+-]*\r?\n?/g, '')
    .replace(/```/g, '')
    .trim();
  const linea = limpio.split('\n').find((l) => l.trim()) ?? limpio;
  return linea.trim();
}
