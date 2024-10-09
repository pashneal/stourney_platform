
<script>
  import Pay from "lucide-svelte/icons/circle-dollar-sign";

  export let size = "30px";
  export let disabled = true;
</script>

<div class="pay {disabled? "disabled" : ""}">
  <Pay size={size}/>
</div>

<style>
  .pay {
    color: green;
    display: flex;
    align-items: center;
  }

  .pay:hover:not(.disabled) {
    cursor: pointer;
    color: darkgreen;
  }

  .disabled {
    color: grey;
  }

</style>
