<script lang="ts">
  import stack from "$lib/assets/card_count.svg";
  export let count : number;
  let max = 7;
  let num = count > max ? max : count;
</script>

<div class="stack-container">
  {#if count > 0}

    <div class="shadow card-stack">
      <div class="count">
        {count}
      </div>
      <img src={stack} alt="Card Stack" draggable="false"/>
    </div>
    {#each Array(num) as _, i}
      <div class="shadow card-stack-{i}">
          <div class="count">
            {count}
          </div>
        <img src={stack} alt="Card Stack" draggable="false"/>
      </div>
    {/each}

  {:else}
    <div class="empty card-stack">
      <img class="hidden" src={stack} alt="Card Stack" draggable="false"/>
    </div>
  {/if}
</div>

<style>
  .stack-container {
    position: relative;
    margin: 0;
    height: 100%;
    width: 100%;
  }

  .empty {
    filter: grayscale(100%);
    border-radius: 20px;
    border: 1px dashed black;
    margin-left: 10%;
  }
  .hidden {
    visibility: hidden;
  }

  .shadow {
    filter: drop-shadow(2px 2px 2px rgba(0, 0, 0, 0.5));
  }

  .card-stack {
    height: 100%;
    width: 100%;
    position: relative;
    display: flex;
  }
  .card-stack-0 {
    height: 100%;
    width: 100%;
    position: absolute;
    bottom: -2px;
    right: -2px;
    z-index: 0;
    display: flex;
  }
  .card-stack-1 {
    height: 100%;
    width: 100%;
    position: absolute;
    bottom: -4px;
    right: -4px;
    z-index: -1;
    display: flex;
  }
  .card-stack-2 {
    height: 100%;
    width: 100%;
    position: absolute;
    bottom: -6px;
    right: -6px;
    z-index: -2;
    display: flex;
  }
  .card-stack-3 {
    height: 100%;
    width: 100%;
    position: absolute;
    bottom: -8px;
    right: -8px;
    z-index: -3;
    display: flex;
  }
  .card-stack-4 {
    height: 100%;
    width: 100%;
    position: absolute;
    bottom: -10px;
    right: -10px;
    z-index: -4;
    display: flex;
  }
  .card-stack-5 {
    height: 100%;
    width: 100%;
    position: absolute;
    bottom: -12px;
    right: -12px;
    z-index: -5;
    display: flex;
  }
  .card-stack-6 {
    height: 100%;
    width: 100%;
    position: absolute;
    bottom: -14px;
    right: -14px;
    z-index: -6;
    display: flex;
  }
  
  img{
    max-width: 100%;
    max-height: 100%;
  }

  .count {
    position: absolute;
    width: 100%;
    height: 100%;
    margin-top: 3%;
    font-size: 3em;
    font-family: 'Zero';
    color: white;
    display: flex;
    padding-left: 10%;

    justify-content: center;
    align-items: center;
  }
</style>
