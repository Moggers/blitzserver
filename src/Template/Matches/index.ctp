<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('New Match'), ['action' => 'add']) ?></li>
    </ul>
</nav>
<div class="matches index large-9 medium-8 columns content">
    <h3><?= __('Matches') ?></h3>
    <table cellpadding="0" cellspacing="0">
        <thead>
            <tr>
                <th><?= $this->Paginator->sort('address') ?></th>
                <th><?= $this->Paginator->sort('map') ?></th>
                <th><?= $this->Paginator->sort('status') ?></th>
                <th><?= $this->Paginator->sort('age') ?></th>
                <th class="actions"><?= __('Actions') ?></th>
            </tr>
        </thead>
        <tbody>
            <?php foreach ($matches as $match): ?>
            <tr>
                <td><?= $this->Number->format($match->port) ?></td>
                <td><?= h($match->map) ?></td>
				<?php if ($match->status == 1) { ?>
					<td><?= "Active" ?></td>
				<?php } else { ?>
					<td><?= "Pending" ?> </td>
				<?php } ?>
				<?php switch( $match->age) {
					case 0: ?>
						<td><?= "Early" ?>
					<?php break;
					case 1: ?>
						<td><?= "Middle" ?>
					<?php break;
					case 2: ?>
						<td><?= "Late" ?>
					<?php break;
				} ?>

                <td class="actions">
                    <?= $this->Html->link(__('View'), ['action' => 'view', $match->id]) ?>
                    <?= $this->Form->postLink(__('Delete'), ['action' => 'delete', $match->id], ['confirm' => __('Are you sure you want to delete # {0}?', $match->id)]) ?>
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
