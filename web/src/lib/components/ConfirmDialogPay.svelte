<script lang="ts">

  import ConfirmDialog from "./ConfirmDialog.svelte"; 
  export let hidden : boolean = false;
  export let iconSize : string = "25px";
  export let payEnabled : boolean = true;

</script>

<ConfirmDialog hidden={hidden} iconSize={iconSize} payEnabled={payEnabled} showPay={true}>
  <slot></slot>
</ConfirmDialog>
