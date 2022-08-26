<script type="ts">
  import BottomBar from './lib/components/BottomBar.svelte';
  import Terminal from './lib/components/Terminal.svelte'
  import { onMount } from 'svelte'

  // onMount(() => {
  //   fetch("http://[ip]/counter/increment")
  //   .then(response => response.json())
  //   .then(data => {
  //     console.log(data)
  //   }).catch(error => {
  //     console.log(error)
  //     return []
  //   })
  // })
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