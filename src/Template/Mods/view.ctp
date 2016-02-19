<div class="mods view large-12 medium-8 columns content">
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
