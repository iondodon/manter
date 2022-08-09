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
    width: 100%;
    height: 100%;
    padding: 0%;
    margin: 0%;
    display: flex;
    flex-direction: column;
  }
</style>