<script lang="ts">
  import Avatar from "./Avatar.svelte"; 

  export let name;
  export let avatar: number = 0;
  export let cards: number = 1;
  export let points: number = 11;
  export let currentPlayer: boolean = false;
</script>

<div class="player-container">
  <Avatar currentPlayer={currentPlayer} avatar={avatar} name={name} points={points} cards={cards} />
  <div class="gems-container">
    <slot></slot>
  </div>
</div>

<style>
  .player-container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    width: 25%;
    height: 20%;
  }
  .gems-container {
    flex-direction: row;
    flex-wrap: wrap;
    display: flex;
    height: 100%;
    gap: 2%;
  }

</style>

