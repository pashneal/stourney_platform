<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Gem from "$lib/components/CardDetailGem.svelte";
  import Row from "$lib/components/CardRow.svelte";
  import { cards , deckCounts} from "$lib/stores/replayStore";



</script>
<div class="cards">
    {#each $cards.reverse() as row, i}
      <Row stackCount={$deckCounts[i]}>
        {#each row as card}
          <Card card_name={card.gem} points={card.points}>
            {#each card.cost.entries() as [gemName, num]}
              <Gem gem_name={gemName} number={num}/>
            {/each}
          </Card>
        {/each}
      </Row>
    {/each}
</div>

<style>
  .cards {
    width: 800px;
    height: 600px;
    left: 0;
    right: 0;
    display: flex;
    flex-direction: column;
    align-items: top;
    justify-content: top;
  }


</style>
