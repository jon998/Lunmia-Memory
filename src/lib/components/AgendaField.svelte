<script lang="ts">
  import { diasInclusiveAgenda, fechaFinEfectiva, type AgendaCivil } from '$lib/agenda';

  interface Props {
    value: AgendaCivil;
    compact?: boolean;
    oninteract?: () => void;
  }
  let { value = $bindable(), compact = false, oninteract }: Props = $props();

  const rango = $derived(diasInclusiveAgenda(value));
  const fin = $derived(fechaFinEfectiva(value));

  function tocar() {
    oninteract?.();
  }

  function cambiarInicio(e: Event) {
    const nueva = (e.currentTarget as HTMLInputElement).value;
    const unDia = value.fechaFin === value.fecha;
    value.fecha = nueva;
    if (unDia || value.fechaFin < nueva) value.fechaFin = nueva;
    tocar();
  }

  function cambiarFin(e: Event) {
    const nueva = (e.currentTarget as HTMLInputElement).value;
    value.fechaFin = nueva < value.fecha ? value.fecha : nueva;
    tocar();
  }
</script>

<div class="agenda" class:compact>
  <label class="check">
    <input
      type="checkbox"
      bind:checked={value.activo}
      onchange={tocar}
    />
    <span>Reunión / recordatorio en el calendario</span>
  </label>
  {#if value.activo}
    <div class="campos">
      <label>
        <span>Inicio</span>
        <input type="date" value={value.fecha} oninput={cambiarInicio} />
      </label>
      <label>
        <span>Fin</span>
        <input type="date" value={value.fechaFin} min={value.fecha} oninput={cambiarFin} />
      </label>
      <label class="todo">
        <input type="checkbox" bind:checked={value.allDay} onchange={tocar} />
        <span>Todo el día</span>
      </label>
      {#if value.allDay}
        <label>
          <span>Aviso</span>
          <input type="time" bind:value={value.hora} oninput={tocar} />
        </label>
      {:else}
        <label>
          <span>De</span>
          <input
            type="time"
            bind:value={value.hora}
            oninput={() => {
              if (value.allDay) value.allDay = false;
              tocar();
            }}
          />
        </label>
        <label>
          <span>A</span>
          <input type="time" bind:value={value.horaFin} oninput={tocar} />
        </label>
      {/if}
    </div>
    <p class="hint">
      {value.fecha}{#if !value.allDay} {value.hora}{/if}
      → {fin}{#if !value.allDay} {value.horaFin}{/if}
      {#if value.allDay} · aviso {value.hora}{/if}
      {#if rango > 1} · {rango} días{/if}
    </p>
  {/if}
</div>

<style>
  .agenda { display: flex; flex-direction: column; gap: 8px; }
  .check {
    display: flex; align-items: center; gap: 8px;
    font-size: 12px; font-weight: 500; cursor: pointer;
  }
  .check input { width: 14px; height: 14px; }
  .campos { display: flex; flex-wrap: wrap; gap: 8px; align-items: end; }
  .campos label { display: flex; flex-direction: column; gap: 3px; }
  .campos span {
    font-family: var(--font-mono); font-size: 10px;
    color: var(--texto-2); letter-spacing: 0.06em;
  }
  .campos input[type="date"],
  .campos input[type="time"] {
    font-family: var(--font-ui); font-size: 12px;
    padding: 5px 8px;
    border: 1px solid var(--borde-fuerte);
    border-radius: var(--radio-m);
    background: var(--superficie); color: var(--tinta);
  }
  .todo {
    flex-direction: row !important;
    align-items: center;
    gap: 6px !important;
    padding-bottom: 6px;
    font-size: 12px;
    cursor: pointer;
  }
  .todo input { width: 14px; height: 14px; }
  .hint { margin: 0; font-family: var(--font-mono); font-size: 10px; color: var(--apagado); }
  .compact .campos { gap: 6px; }
</style>
