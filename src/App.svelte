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
    background-color: black;
    overflow: hidden;
  }

  main {
    display: flex;
    flex-direction: column;
    height: 99.8%;
    width: 99.8%;
    margin: auto;

    overflow: hidden;
  }
</style>