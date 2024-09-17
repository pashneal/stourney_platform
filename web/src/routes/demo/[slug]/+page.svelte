<script lang="ts">
  import CardDislay from "$lib/components/CardDisplay.svelte";
  import GemToken from "$lib/components/GemToken.svelte";
  import Bank from "$lib/components/Bank.svelte";
  import Noble from "$lib/components/Noble.svelte";
  import NobleDetail from "$lib/components/NobleDetails.svelte";
  import GemTokenSmall from "$lib/components/GemTokenSmall.svelte"; 
  import VDivider from "$lib/components/VerticalDivider.svelte";
  import HDivider from "$lib/components/HorizontalDivider.svelte";
  import Player from "$lib/components/Player.svelte";

  import { onMount } from "svelte";

  import { turnNumber, nobles, bank, players, updateGameDeckCounts, updateGamePlayers, updateGameNobles , updateGameBanks, updateGameCards} from "$lib/stores/replayStore"; 
  import type { GameBackendDesc } from "$lib/stores/replayStore";



  function nextMove() {
    turnNumber.update(n => n + 1);
  }

  function prevMove() {
    turnNumber.update(n => n - 1);
  }

  function updateMoveInput(move: number) {
    turnNumber.set(move);
  }

  let moveInput = 0;

  export let data;

  function refreshBoard(update : GameBackendDesc) {
    console.log(update);
    updateGamePlayers(update, moveInput);
    updateGameNobles(update, moveInput);
    updateGameBanks(update, moveInput);
    updateGameCards(update, moveInput);
    updateGameDeckCounts(update, moveInput);
  }

  function getGameDesc(move: number) {
    if (data.cache.has(move)) {
      console.log("using cache move: " + move); 
      refreshBoard(data.cache.get(move)!);
      return;
    }
    console.log("fetching move: " + move);
    fetch("/api", 
      {
        method : "POST", 
        body : JSON.stringify({"uuid": data.slug, "turnNumber" : move }),  
        headers: {"Content-type": "application/json; charset=UTF-8"}
      }
    ).then((r) => r.json())
     .then(r => {
       r = r.success.game_update as GameBackendDesc;
       data.cache.set(move, r);
       refreshBoard(r);
       return r
     });
  }

  onMount(() => {
    turnNumber.subscribe(value => {
      moveInput = value;
      getGameDesc(value);

    });
  });


</script>


<svelte:head>
	<title>Demo</title>
	<meta name="description" content="A demonstration of the the stourney app running a splendor game" />
</svelte:head>

<div class="top-bar">
  <button on:click={prevMove}>{"<"}</button>
  <input type="number"  id="moveInput" bind:value={moveInput} on:change={() => turnNumber.update(() => moveInput)}/>
  <button on:click={nextMove}>{">"}</button>
</div>

<div class="game">
  <div class="game-inner">
    <VDivider/>
    <Bank>
      {#each $bank as bankDesc}
        <GemToken tokenName={bankDesc.gemName} numRemaining={bankDesc.gemCount} />
      {/each}
    </Bank>
    <VDivider/>
    <div>
      <CardDislay/>
    </div>
    <VDivider/>
    <div class="nobles">
      {#each $nobles as noble}
        <Noble>
          {#each noble.requirements as req}
            <NobleDetail number={req.gemCount} gem_name={req.gemName} />
          {/each}
        </Noble>
      {/each}
    </div>
    <VDivider/>
    
  </div>

  <HDivider/>

  <div class="players">
    {#each $players as player, index}
      <Player currentPlayer={player.currentPlayer} avatar={index} name={player.name} points={player.totalPoints} cards={player.numReservedCards} >
        {#each player.gems.entries() as [gemName, gemCount]}
          <GemTokenSmall tokenName={gemName} numRemaining={gemCount} cardCount={player.developments.get(gemName)} />
        {/each}
      </Player>
    {/each}
  </div>

</div>


<style>
  .game {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
  }
  .game-inner {
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    width: 70rem;
    height: 40rem;
  }

  .nobles {
    display: flex;
    flex-direction: column;
    align-items: top;
    gap: 5%;
    width: 20%;
    height: 70%;
  }

  .players {
    flex-direction : row;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 80%;
  }
  .top-bar{
    position: fixed;
    top: 0;
  }

</style>
