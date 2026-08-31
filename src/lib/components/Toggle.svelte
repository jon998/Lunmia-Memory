<script lang="ts">
  interface Props {
    checked: boolean;
    onchange?: (value: boolean) => void;
    label?: string;
  }
  let { checked = $bindable(), onchange, label }: Props = $props();
</script>

<button
  type="button"
  role="switch"
  aria-checked={checked}
  aria-label={label}
  class="track"
  class:on={checked}
  onclick={() => { checked = !checked; onchange?.(checked); }}
>
  <span class="thumb"></span>
</button>

<style>
  .track {
    width: 36px;
    height: 21px;
    border-radius: 999px;
    background: var(--borde-fuerte);
    border: none;
    padding: 0;
    position: relative;
    transition: background 0.15s ease;
    flex-shrink: 0;
  }
  .track.on { background: var(--indigo); }
  .thumb {
    position: absolute;
    top: 2.5px;
    left: 3px;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: var(--superficie);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.25);
    transition: left 0.15s ease;
  }
  .on .thumb { left: 17px; }
</style>
