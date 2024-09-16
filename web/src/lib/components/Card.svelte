<script lang="ts">
  import emerald_card from "$lib/assets/emerald_card.svg";
  import sapphire_card from "$lib/assets/sapphire_card.svg";
  import diamond_card from "$lib/assets/diamond_card.svg";
  import ruby_card from "$lib/assets/ruby_card.svg";
  import onyx_card from "$lib/assets/onyx_card.svg";

  import zero from "$lib/assets/0.svg";
  import one from "$lib/assets/1.svg";
  import two from "$lib/assets/2.svg";
  import three from "$lib/assets/3.svg";
  import four from "$lib/assets/4.svg";
  import five from "$lib/assets/5.svg";
  import six from "$lib/assets/6.svg";
  import seven from "$lib/assets/7.svg";

  export let points : number = 0;
  export let card_name : "emerald" | "sapphire" | "ruby" | "diamond"  | "onyx";

  let cards = {
    "emerald": emerald_card,
    "sapphire": sapphire_card,
    "diamond": diamond_card,
    "ruby": ruby_card,
    "onyx": onyx_card
  }

  let nums = [zero, one, two, three, four, five, six, seven];

</script>

<div class="card">
  <img src={cards[card_name]} alt="Card" draggable="false"/>
  <div class="card-details"> 
    <slot></slot>
  </div>
  <div class="points">
    <img src={nums[points]} alt="points" draggable="false"/> 
  </div>
</div>

<style>

  .card {
    max-width: 20%;
    height: 95%;
    -khtml-user-select: none;
    -o-user-select: none;
    -moz-user-select: none;
    -webkit-user-select: none;
    user-select: none;
    position: relative;
  }

  .card:hover {
    transform: scale(1.1);
    z-index: 1;
    filter: drop-shadow(2px 2px 2px rgba(0, 0, 0, 0.5));
  }

  .card {
    transition: transform 0.1s;
  }
  
  img{
    max-width: 100%;
    max-height: 100%;
  }

  .card-details {
    top: 0%;
    left: 0;
    right: 0;
    max-width: 90%;
    height: 50%;
    margin-inline: auto;
    position: absolute;
  }

  .points {
    font-family: "Zero";
    position: absolute;
    height: 20%;
    left: 75%;
    bottom: 5%;
    color: white;
    width: 25%;
    object-fit: scale-down;
  }

</style>
