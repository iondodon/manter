<script type="ts">
  import BottomBar from './lib/components/BottomBar.svelte';
  import Terminal from './lib/components/Terminal.svelte'
</script>

<main>
    <Terminal/>
    <BottomBar/>
</main>

<style type="scss">
  :root {
    font-family: monospace;
    padding: 0%;
    margin: 0%;
    border: 0%;

    background-color: yellow;
  }

  main {
    display: flex;
    flex-direction: column;
    height: 99.8%;
    width: 99.8%;
    margin: auto;

    border: 0%;
    padding: 0%;

    overflow-y: hidden;
    overflow-x: hidden;
  }
</style>