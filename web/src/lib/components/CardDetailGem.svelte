<script lang="ts"> 
  import emerald from "$lib/assets/icon1_emerald.svg";
  import sapphire from "$lib/assets/icon1_sapphire.svg";
  import diamond from "$lib/assets/icon1_diamond.svg";
  import ruby from "$lib/assets/icon1_ruby.svg";
  import onyx from "$lib/assets/icon1_onyx.svg";


  export let gem_name : "emerald" | "sapphire" | "ruby" | "diamond"  | "onyx";
  export let number : number;

  let gems = {
    "emerald": emerald,
    "sapphire": sapphire,
    "diamond": diamond,
    "ruby": ruby,
    "onyx": onyx
  }
</script>

<div class="card-detail-container">
  <img src={gems[gem_name]} alt="Gem" draggable="false"/>
  <div class="pip-container"> 
    <span class="pip-buffer"></span>
    {#each Array(number) as _}
      <div class="pip {gem_name}">
      </div>
    {/each}
  </div>
</div>



<style>
  .card-detail-container {
    top : 5%;
    width: 100%;
    height: 20%;
    position : relative;
    background-color: var(--color-pip-container);
    border-radius: 20px;
    margin-bottom: 5%;
  }

  img{
    width: 30%;
    height: 120%;
    position: absolute;
    left: -5%;
    top: -10%;
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
  }
  .pip-container{
    display: flex;
    justify-content: left;
    gap: 4%;
    position: absolute;
    width: 100%;
    height: 100%;
    align-items: center;
  }

  .pip-buffer {
    width: 20%;
  }
  .pip{
    border-radius: 50%;
    width: 6px;
    height:  6px;
  }

  .onyx.pip{
    background-color: var(--color-onyx-pip);
  }

  .diamond.pip{
    background-color: var(--color-diamond-pip);
  }

  .emerald.pip{
    background-color: var(--color-emerald-pip);
  }

  .ruby.pip{
    background-color: var(--color-ruby-pip);
  }

  .sapphire.pip{
    background-color: var(--color-sapphire-pip);
  }

  

</style>
