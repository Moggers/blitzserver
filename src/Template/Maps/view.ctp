<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('Edit Map'), ['action' => 'edit', $map->id]) ?> </li>
        <li><?= $this->Form->postLink(__('Delete Map'), ['action' => 'delete', $map->id], ['confirm' => __('Are you sure you want to delete # {0}?', $map->id)]) ?> </li>
        <li><?= $this->Html->link(__('List Maps'), ['action' => 'index']) ?> </li>
        <li><?= $this->Html->link(__('New Map'), ['action' => 'add']) ?> </li>
    </ul>
</nav>
<div class="maps view large-9 medium-8 columns content">
    <h3><?= h($map->name) ?></h3>
    <table class="vertical-table">
        <tr>
			<th><?= __('Description') ?></th>
            <th><?= h($map->description) ?></td>
        </tr>
        <tr>
			<th><?= __('Provinces(Sea Provinces)')?> </th>
			<th><?= h($map->prov.'('.$map->seaprov) .')' ?> </td>
        </tr>
		<tr>
			<th><?=$this->Html->image( 'maps/' . $map->id . '/thumb512.jpeg', ['alt' => 'CakePHP' ]) ?></th>
		</tr>
    </table>
</div>
