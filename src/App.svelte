<script type="ts">
  import BottomBar from './lib/components/BottomBar.svelte';
  import Terminal from './lib/components/Terminal.svelte'
  import { onMount } from 'svelte'

  onMount(() => {
    fetch("http://137.184.207.126:8080/counter/increment", { method: 'PUT' })
    .then(response => response.json())
    .then(_data => {
    }).catch(_error => {
      return []
    })
  })
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
    height: 100%;
    width: 100%;
    margin: auto;
    overflow: hidden;
  }
</style>