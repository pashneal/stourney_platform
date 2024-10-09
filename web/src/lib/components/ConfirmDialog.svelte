<script lang="ts">
  import Accept from "$lib/components/Accept.svelte";
  import Cancel from "$lib/components/Cancel.svelte";
  import Pay from "$lib/components/Pay.svelte";

  export let hidden : boolean = false;
  export let iconSize : string = "25px";
  export let acceptEnabled : boolean = true;
  export let payEnabled : boolean = true;

  export let showAccept : boolean = false;
  export let showPay : boolean = false;
</script>

<div class="confirm-container {hidden? "hidden" : ""}">
  <slot></slot>
  <div class="options">
    {#if showAccept}
      <Accept size={iconSize} disabled={!acceptEnabled}/>
    {/if}
    {#if showPay}
      <Pay size={iconSize} disabled={!payEnabled}/>
    {/if}
    <Cancel size={iconSize}/>
  </div>
</div>


<style>
  .confirm-container {
    width: 100%;
    position: relative;
    border-radius: 20px;
  }

  .options {
    position: relative;
    border-radius: 20px;
    background-color: white;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 10px;
    margin-top: 10px;
    z-index: 2;
  }

  .hidden .options {
    visibility: hidden;
  }

</style>
