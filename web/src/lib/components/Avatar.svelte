<script lang="ts">
  import av1 from "$lib/assets/av1.svg";
  import av2 from "$lib/assets/av2.svg";
  import av3 from "$lib/assets/av3.svg";
  import av4 from "$lib/assets/av4.svg";
  import av5 from "$lib/assets/av5.svg";
  import av6 from "$lib/assets/av6.svg";
  import av7 from "$lib/assets/av7.svg";
  import av8 from "$lib/assets/av8.svg";
  import av9 from "$lib/assets/av9.svg";

  export let avatar : number = 0;
  export let name : string = "Player " + avatar;
  export let cards: number = 0;
  export let points : number = 10;
  export let currentPlayer : boolean = false;

  let avatars = [av1, av2, av3, av4, av5, av6, av7, av8, av9];
</script>

<div class="avatar-container">
  <div class="avatar {(currentPlayer) ? 'highlight' : ''}">
    <img src={avatars[avatar]} alt="avatar" draggable="false"/>
  </div>
  <p class="name">{name}</p>
  <div class="points">points: {points}</div>
  <div class="cards">cards: {cards} </div>
</div>

<style>
  div {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100px;
    height: 100px;
  }

  .name {
    border-bottom: 1px dashed black;
    font-family: 'Quicksand Book';
    font-weight: bold;
  }
  .points {
    height: 20px;
    padding: 0;
    font-family: 'Quicksand Book';
    font-weight: bold;
  }
  .cards {
    height: 20px;
    padding: 0;
    font-family: 'Quicksand Book';
    font-weight: bold;
  }
  .avatar {
    width : 100%;
    height: 50%;
  }
  img {
    width: 100%;
    height: 100%;
  }

  .highlight{
    background-color: #f0f0f0;
    border-radius: 10px;
  }
</style>

