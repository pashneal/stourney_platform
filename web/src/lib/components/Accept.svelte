<script>
  import Accept from "lucide-svelte/icons/circle-check";

  export let size = "30px";
  export let disabled = true;
</script>

<div class="accept {disabled? "disabled" : ""}">
  <Accept size={size}/>
</div>

<style>
  .accept {
    color: green;
    display: flex;
    align-items: center;
  }

  .accept:hover:not(.disabled) {
    cursor: pointer;
    color: darkgreen;
  }

  .disabled {
    color: grey;
  }

</style>
