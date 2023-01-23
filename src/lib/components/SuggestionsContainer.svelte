<script lang="ts">
    export let suggestions: object[]  = [];
</script>

<ol id="suggestions-container">
    {#each suggestions as suggestion}
        <li>{JSON.stringify(suggestion)}</li>
    {/each}
</ol>

<style lang="scss">
    #suggestions-container {
        position: absolute;
        top: 0;
        right: 0;
        width: wrap-content;
        background-color: rgb(101, 101, 101);
        color: #fff;
        list-style: none;
        padding: 0;
        margin: 0;
    }
</style>