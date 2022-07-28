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
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    padding: 0%;
    margin: 0%;
  }

  main {
    display: flex;
    flex-direction: column;
    margin: 0%;
    padding: 0%;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    bottom: 0%;
  }
</style>