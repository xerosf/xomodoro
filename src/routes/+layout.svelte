<script lang="ts">
  import TitleBar from '../lib/TitleBar.svelte';
</script>

<TitleBar />
<main class="app-content">
  <slot />
</main>

<style>
  .app-content {
    margin-top: 28px; /* Account for titlebar height */
    overflow: auto;
  }
</style>