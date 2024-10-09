<script>
  import ConfirmDialogAccept from "$lib/components/ConfirmDialogAccept.svelte";
</script>

<div class="gems">
  <ConfirmDialogAccept>
    <slot></slot>
  </ConfirmDialogAccept>
</div>


<style>
  .gems {
    display: flex;
    justify-content: center;
    flex-direction: column;
    flex-wrap: wrap;
    width: 10%;
    gap: 5px;
  }

</style>
