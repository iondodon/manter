<script lang="ts">

</script>

<div id="bottom-bar">

  <!-- counterAPI library, insert anywhere on the page. -->
  <script src="https://counterapi.com/c.js" async ></script>
  <!-- Add the counter instance somewhere to your template... -->
  <div class="counterapi" noIcon="true" noCss="true"  ></div>
  <span> global usages</span>


  <span> | </span>
  <div class="version">
    Manter (alpha)
  </div>
</div>

<style lang="scss">
  #bottom-bar {
    display: flex;
    flex-direction: row;
    justify-content: end;

    bottom: 0em;
    height: 1.2em;
    width: 100%;
    background-color: rgb(29, 29, 29);
    color: rgb(156, 156, 156);
  }

  #bottom-bar * {
    margin-left: 0.2rem;
    margin-right: 0.2rem;
  }
</style>