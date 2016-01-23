<script src="https://code.jquery.com/jquery-2.2.0.min.js" type='text/javascript'></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/lodash.js/4.0.0/lodash.js" type='text/javascript'></script>
<script type='text/javascript' >
	$(document).ready( function() {
		setInterval( function() { 
			$.get( window.location.pathname + '?layout=false', function( data, res ) {  
				$(".matches table" ).html($("table", data));
			} );
		}, 5000 );
	} );
</script>
<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('Request New Match'), ['action' => 'add']) ?></li>
        <li><?= $this->Html->link(__('List Maps'), ['controller' => 'Maps', 'action' => 'index']) ?></li>
        <li><?= $this->Html->link(__('Upload Map'), ['controller' => 'Maps', 'action' => 'add']) ?></li>
    </ul>
</nav>
<div class="matches index large-9 medium-8 columns content">
    <h3><?= __('Matches') ?></h3>
    <table cellpadding="0" cellspacing="0">
        <thead>
            <tr>
                <th><?= $this->Paginator->sort('name') ?></th>
                <th><?= $this->Paginator->sort('map_id') ?></th>
                <th><?= $this->Paginator->sort('age') ?></th>
				<th><?= $this->Paginator->sort('port') ?></th>
				<th><?= $this->Paginator->sort('status') ?></th>
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
