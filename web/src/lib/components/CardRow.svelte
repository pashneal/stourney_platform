<script>
  import CardStack from "./CardStack.svelte"; 
  export let stackCount = 0;
</script>

<div class="display">
  <div class="cards">
    <slot></slot>
  </div>
  <div class="stack">
    <CardStack count={stackCount}/>
  </div>
</div>

<style>
  .display {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 30%;
    width: 100%;
    position: relative;
  }
  .stack {
    width: 12%;
    position: relative;
  }

  .cards {
    display: flex;
    gap: 1rem;
    row-gap: 1rem;
    width: 80%;
    height: 100%;
    justify-content: left;
  }

</style>
