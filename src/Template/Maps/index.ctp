<script type='text/javascript' >
	function swapImage(id, mid, size)
	{
		document.getElementById( id + 'mapimage' ).src = 'img/maps/' + mid + '/thumb'+size+'.jpeg';
	}
</script>
<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
    </ul>
</nav>
<div class="maps index large-9 medium-8 columns content">
    <h3><?= __('Maps') ?></h3>
    <table>
        <thead>
            <tr>
                <th><?= $this->Paginator->sort('Name') ?></th>
                <th><?= $this->Paginator->sort('Description') ?></th>
                <th><?= $this->Paginator->sort('Province Count') ?></th>
				<th><?= $this->Paginator->sort('Actions') ?> </th>
            </tr>
        </thead>
        <tbody>
            <?php foreach ($maps as $map): ?>
            <tr>
					<td><?= $this->Html->image( 'maps/' . $map->id . '/thumb64.jpeg', [ 
						'id' => $map->id . 'mapimage', 
						'onmouseover' => 'swapImage('.$map->id.','.$map->id.',256'.')', 
						'onmouseout' =>  'swapImage('.$map->id.','.$map->id.',64'.')',
						'alt' => 'CakePHP' ]) ?> </td>
                <?= h($map->name) ?></td>
				<td><?= h($map->description) ?></td>
                <td><?= h($map->prov . '(' . $map->seaprov) . ')' ?></td>
				<?= $this->html->link(__('View'), ['action' => 'view', $map->id]) ?> </td>
            </tr>
            <?php endforeach; ?>
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
