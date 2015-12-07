<nav class="large-3 medium-4 columns" id="actions-sidebar">
    <ul class="side-nav">
        <li class="heading"><?= __('Actions') ?></li>
        <li><?= $this->Form->postLink(__('Delete Match'), ['action' => 'delete', $match->id], ['confirm' => __('Are you sure you want to delete # {0}?', $match->id)]) ?> </li>
        <li><?= $this->Html->link(__('List Matches'), ['action' => 'index']) ?> </li>
        <li><?= $this->Html->link(__('New Match'), ['action' => 'add']) ?> </li>
    </ul>
</nav>
<div class="matches view large-9 medium-8 columns content">
    <h3><?= h($match->id) ?></h3>
    <table class="vertical-table">
        <tr>
            <th><?= __('Map') ?></th>
            <td><?= h($match->map) ?></td>
        </tr>
        <tr>
            <th><?= __('Id') ?></th>
            <td><?= $this->Number->format($match->id) ?></td>
        </tr>
        <tr>
            <th><?= __('Port') ?></th>
            <td><?= $this->Number->format($match->port) ?></td>
        </tr>
        <tr>
            <th><?= __('Status') ?></th>
            <td><?= $this->Number->format($match->status) ?></td>
        </tr>
    </table>
</div>
