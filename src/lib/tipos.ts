export const TIPOS_POR_DEFECTO = ['bug', 'script', 'idea', 'nota', 'recordatorio'] as const;

export type TipoPorDefecto = (typeof TIPOS_POR_DEFECTO)[number];

export type TipoChipVariant = 'indigo' | 'alerta' | 'ok' | 'neutral' | 'error';

export const TIPO_ESTILO: Record<string, TipoChipVariant> = {
  bug: 'error',
  script: 'ok',
  idea: 'indigo',
  nota: 'neutral',
  tarea: 'neutral',
  recordatorio: 'alerta',
  recomendación: 'ok'
};

export function estiloTipo(nombre: string | null | undefined): TipoChipVariant {
  if (!nombre) return 'neutral';
  return TIPO_ESTILO[nombre] ?? 'neutral';
}

export function tituloEntrada(contenido: string): string {
  return contenido.split('\n')[0]?.trim() || contenido;
}

export function fusionarTipos(extras: string[] = []): string[] {
  const vistos = new Set<string>(TIPOS_POR_DEFECTO);
  const extra: string[] = [];
  for (const raw of extras) {
    const t = raw.trim().toLowerCase();
    if (!t || vistos.has(t)) continue;
    vistos.add(t);
    extra.push(t);
  }
  return [...TIPOS_POR_DEFECTO, ...extra];
}
