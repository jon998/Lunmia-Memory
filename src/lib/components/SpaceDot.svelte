<script lang="ts">
  interface Props { color: string; size?: number }
  let { color, size = 8 }: Props = $props();
</script>

<span class="dot" style:background={color} style:width="{size}px" style:height="{size}px"></span>

<style>
  .dot {
    display: inline-block;
    border-radius: 2px;
    flex-shrink: 0;
  }
</style>
