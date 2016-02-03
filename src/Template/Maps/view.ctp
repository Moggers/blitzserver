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
	<div class="wrapper">
		<div class="maptable">
			<table class="vertical-table" style="background:#fafafa">
				<td style="vertical-align: top">
				<table class="vertical-table" style="background:#fafafa">
					<tr>
						<th><?= __('Description') ?></th>
						<td><?= h($map->description) ?></td>
					</tr>
					<tr>
						<th><?= __('Provinces(Sea Provinces)')?> </th>
						<td><?= h($map->prov.'('.$map->seaprov) .')' ?> </td>
					</tr>
					<tr>
					</tr>
				</table>
				</td>
			</table>
		</div>
		<div class="mappreview">
			<td><?=$this->Html->image( 'maps/' . $map->id . '/thumb512.jpeg', ['alt' => 'CakePHP' ]) ?></td>
		</div>
	</div>
	</table>
</div>
