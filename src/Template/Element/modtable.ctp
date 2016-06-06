    <table cellpadding="0" cellspacing="0" id='modtable'>
        <thead>
            <tr>
                <th style="width:200px"><?= $this->Paginator->sort('name') ?></th>
                <th style="width:276px"><?= $this->Paginator->sort('icon') ?></th>
                <th style="width:100px"><?= $this->Paginator->sort('version') ?></th>
                <th><?= $this->Paginator->sort('description') ?></th>
            </tr>
        </thead>
        <tbody>
            <?php foreach ($mods as $mod): ?>
			<tr clas="modpane" id=<?= 'mod_'.$mod->id ?>>
				<td><?= h($mod->name) ?></td>
				<td><?= $this->Html->link( $this->Html->image( 'mods/' . $mod->id . '/thumb64.jpeg'), ['controller' => 'Mods', 'action' => 'view', $mod->id], ['escape' => false]) ?></td>
				<td><?= $mod->version ?></td>
				<td class="poop"><?= h($mod->description) ?></td>
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
