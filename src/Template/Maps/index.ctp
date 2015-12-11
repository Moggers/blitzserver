<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('New Map'), ['action' => 'add']) ?></li>
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
            </tr>
        </thead>
        <tbody>
            <?php foreach ($maps as $map): ?>
            <tr>
				<td><?= $this->Html->image( 'maps/' . $map->id . '/thumb64.jpeg', ['alt' => 'CakePHP' ]); ?>
                <?= h($map->name) ?></td>
                <td><?= h($map->description) ?></td>
                <td><?= h($map->prov . '(' . $map->seaprov) . ')' ?></td>
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
