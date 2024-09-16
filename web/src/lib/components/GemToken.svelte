<script lang="ts">
  import emerald from "$lib/assets/token_emerald.svg";
  import sapphire from "$lib/assets/token_sapphire.svg";
  import diamond from "$lib/assets/token_diamond.svg";
  import ruby from "$lib/assets/token_ruby.svg";
  import onyx from "$lib/assets/token_onyx.svg";
  import gold from "$lib/assets/token_gold.svg";


  export let tokenName : "emerald" | "sapphire" | "ruby" | "diamond"  | "onyx" | "gold";
  export let numRemaining : number;

  let tokens = {
    "emerald": emerald,
    "sapphire": sapphire,
    "diamond": diamond,
    "ruby": ruby,
    "onyx": onyx,
    "gold": gold
  }

</script>

<div class="token-container">

  {#each Array(numRemaining) as _,i}
    {#if i == numRemaining - 1}
      <div class="top token-float-{i}">
        <img src={tokens[tokenName]} alt="Token" draggable="false"/>
      </div>
    {:else}
      <div class="token-float-{i}">
        <img src={tokens[tokenName]} alt="Token" draggable="false"/>
      </div>
    {/if}
  {/each}

  <div class="num-remaining">
    {numRemaining}
  </div>
</div>


<style>
  .token-container {
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
    text-align:center;
    width: 100px;
    height: 50px;
    border-radius: 20px;
    border: 1px dashed black;
    display: flex;
    flex-direction: row;
    position: relative;
  }

  img{
    width:40px;
    height:40px;
    margin: 5px;
  }

  .num-remaining {
    font-size: 30px;
    font-family: 'Zero';
    padding-left: 30%;
    margin: auto;
  }

  .top:hover {
    transform: translate(-5px, -5px);
    z-index: 1;
    filter: drop-shadow(2px 2px 10px rgba(255,255, 255, 0.5));
    transition: transform 0.1s;
  }


  .token-float-0 {
    position: absolute;
  }
  .token-float-1 {
    position: absolute;
    top: -2px;
    left: -2px;
  }
  .token-float-2 {
    position: absolute;
    top: -4px;
    left: -4px;
  }
  .token-float-3 {
    position: absolute;
    top: -6px;
    left: -6px;
  }
  .token-float-4 {
    position: absolute;
    top: -8px;
    left: -8px;
  }
  .token-float-5 {
    position: absolute;
    top: -10px;
    left: -10px;
  }
  .token-float-6 {
    position: absolute;
    top: -12px;
    left: -12px;
  }
  /* animate the top token to transform up and left a bit*/
  .top {
    transition: transform 0.2s;
  }

</style>

