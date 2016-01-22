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
            <?php foreach ($matches as $match): ?>
            <tr>
                <td><?= $match->name ?></td>
                <td><?= $match->has('map') ? $this->Html->link(
					$this->Html->image( 'maps/' . $match->map->id . '/thumb64.jpeg', ['alt' => 'CakePHP' ]), 
					['controller' => 'Maps', 'action' => 'view', $match->map->id], 
					['escape' => false]) : '' ?> </td>
                <td><?= $match::ages( $match->age ) ?></td>
				<?php if ($match->status < 1 ): ?>
					<td><?= 'N/A' ?></td>
				<?php else: ?>
					<td><?= $this->Number->format($match->port) ?></td>
				<?php endif;?>
                <td><?= $match::statuses( $match->status ) ?></td>
				<td>
				<?php if ($match->status < 2 ): ?>
					<?= $this->Html->link(__('Start Game'), ['action' => 'start', $match->id]) ?> <br />
				<?php endif; ?>
				<?= $this->Html->link(__('KILL THE GAME'), ['action' => 'destroy', $match->id]) ?> </td>
                <td class="actions">
                </td>
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
