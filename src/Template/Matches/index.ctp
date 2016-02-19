<script type='text/javascript' >
	var waitExtend = 5;
	var refreshmeme = 5;
	function refresh()
	{
		$.get( window.location.pathname + '?layout=false', function( data, res ) {  
			$(".matches table" ).html($("table", data));
			refreshmeme = waitExtend;
		});
	}
	$(document).ready( function() {
		setInterval( function() { 
			refreshbutton = document.getElementById( 'refresh' );
			refreshbutton.innerHTML = 'Refresh (' + refreshmeme + ')'
			refreshmeme = refreshmeme - 1;
			if( refreshmeme == 0 ) {
				waitExtend = waitExtend * 2;
				refresh();
				return;
			}
		}, 1000 );
	} );
</script>
<div class="matches index large-12 medium-8 columns content">
    <h3><?= __('Matches') ?></h3>
	<?= $this->Form->Button( "Refresh", array( 'onclick' => 'refresh(); waitExtend = 5;', 'id' => 'refresh', 'style' => 'padding: 0.5rem 0.5rem 0.5rem 0.5rem' ) ); ?>
	<?= $this->element( 'matchtable', array( 'matches' => $matches ) ); ?>
    <div class="paginator">
        <ul class="pagination">
            <?= $this->Paginator->prev('< ' . __('previous')) ?>
            <?= $this->Paginator->numbers() ?>
            <?= $this->Paginator->next(__('next') . ' >') ?>
        </ul>
        <p><?= $this->Paginator->counter() ?></p>
    </div>
</div>
