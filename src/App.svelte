<script type="ts">
  import BottomBar from './lib/components/BottomBar.svelte';
  import Terminal from './lib/components/Terminal.svelte'
</script>

<main>
    <Terminal/>
    <BottomBar/>
</main>

<style type="scss">
  main {
    height: 100%;
    overflow-y: hidden;
    width: 100%;
    padding: 0;
    margin: 0;
  }
</style>