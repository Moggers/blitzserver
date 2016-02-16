<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Html->link(__('Edit Mod'), ['action' => 'edit', $mod->id]) ?> </li>
        <li><?= $this->Form->postLink(__('Delete Mod'), ['action' => 'delete', $mod->id], ['confirm' => __('Are you sure you want to delete # {0}?', $mod->id)]) ?> </li>
        <li><?= $this->Html->link(__('List Mods'), ['action' => 'index']) ?> </li>
        <li><?= $this->Html->link(__('New Mod'), ['action' => 'add']) ?> </li>
    </ul>
</nav>
<div class="mods view large-9 medium-8 columns content">
    <h3><?= h($mod->name) ?></h3>
    <table class="vertical-table">
        <tr>
            <th><?= __('Name') ?></th>
            <td><?= h($mod->name) ?></td>
        </tr>
        <tr>
            <th><?= __('Icon') ?></th>
            <td><?= h($mod->icon) ?></td>
        </tr>
        <tr>
            <th><?= __('Description') ?></th>
            <td><?= h($mod->description) ?></td>
        </tr>
        <tr>
            <th><?= __('Id') ?></th>
            <td><?= $this->Number->format($mod->id) ?></td>
        </tr>
        <tr>
            <th><?= __('Version') ?></th>
            <td><?= $this->Number->format($mod->version) ?></td>
        </tr>
    </table>
</div>
