<?= $this->Html->script('https://code.jquery.com/jquery-2.2.0.min.js'); ?>
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
				console.log( 'hey fag' );
				waitExtend = waitExtend * 2;
				refresh();
				return;
			}
		}, 1000 );
	} );
</script>
<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
    </ul>
</nav>
<div class="matches index large-9 medium-8 columns content">
    <h3><?= __('Matches') ?></h3>
	<?= $this->Form->Button( "Refresh", array( 'onclick' => 'refresh(); waitExtend = 5;', 'id' => 'refresh', 'style' => 'padding: 0.5rem 0.5rem 0.5rem 0.5rem' ) ); ?>
    <table cellpadding="0" cellspacing="0">
        <thead>
            <tr>
                <th><?= $this->Paginator->sort('name') ?></th>
                <th><?= $this->Paginator->sort('map_id') ?></th>
                <th><?= $this->Paginator->sort('age') ?></th>
				<th><?= $this->Paginator->sort('port') ?></th>
				<th><?= $this->Paginator->sort('status') ?></th>
				<th><?= $this->Paginator->sort('thrones') ?></th>
				<th><?= $this->Paginator->sort('action') ?></th>
            </tr>
        </thead>
        <tbody>
			<?= $this->element( 'matchtable', array( 'matches' => $matches ) ); ?>
        </tbody>
    </table>
    <div class="paginator">
        <ul class="pagination">
            <?= $this->Paginator->prev('< ' . __('previous')) ?>
            <?= $this->Paginator->numbers() ?>
            <?= $this->Paginator->next(__('next') . ' >') ?>
        </ul>
        <p><?= $this->Paginator->counter() ?></p>
    </div>
</div>
