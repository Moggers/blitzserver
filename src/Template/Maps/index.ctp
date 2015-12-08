<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('New Map'), ['action' => 'add']) ?></li>
    </ul>
</nav>
<div class="maps index large-9 medium-8 columns content">
    <h3><?= __('Maps') ?></h3>
    <table cellpadding="0" cellspacing="0">
        <thead>
            <tr>
                <th><?= $this->Paginator->sort('Name') ?></th>
                <th class="actions"><?= __('Actions') ?></th>
            </tr>
        </thead>
        <tbody>
            <?php foreach ($maps as $map): ?>
            <tr>
                <td><?= h($map->name) ?></td>
				<td><?= $this->Html->image($map->name + '.jpg', ['alt' => 'CakePHP' ]); ?> </td>
                <td class="actions">
                    <?= $this->Html->link(__('View'), ['action' => 'view', $map->id]) ?>
                    <?= $this->Html->link(__('Edit'), ['action' => 'edit', $map->id]) ?>
                    <?= $this->Form->postLink(__('Delete'), ['action' => 'delete', $map->id], ['confirm' => __('Are you sure you want to delete # {0}?', $map->id)]) ?>
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
