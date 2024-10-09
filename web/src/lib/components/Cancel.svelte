<script>
  import Cancel from "lucide-svelte/icons/circle-x";

  export let size = "30px";
</script>

<div class="cancel">
  <Cancel size={size}/>
</div>

<style>
  .cancel {
    color: red;
    display: flex;
    align-items: center;
  }

  .cancel:hover {
    cursor: pointer;
    color: darkred;
  }

</style>
