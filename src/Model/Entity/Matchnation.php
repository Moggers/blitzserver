<?php
namespace App\Model\Entity;

use Cake\ORM\Entity;

/**
 * Matchnation Entity.
 *
<<<<<<< HEAD
 * @property int $matchid
 * @property int $nationid
 * @property int $markdelete
=======
 * @property int $id
 * @property int $nation_id
 * @property \App\Model\Entity\Nation $nation
 * @property int $match_id
 * @property \App\Model\Entity\Match $match
>>>>>>> Transitioned from bitstring based nation handling to a discrete table with a belongsToMany relationship
 */
class Matchnation extends Entity
{

    /**
     * Fields that can be mass assigned using newEntity() or patchEntity().
     *
     * Note that when '*' is set to true, this allows all unspecified fields to
     * be mass assigned. For security purposes, it is advised to set '*' to false
     * (or remove it), and explicitly make individual fields accessible as needed.
     *
     * @var array
     */
    protected $_accessible = [
        '*' => true,
        'match_id' => false,
        'nation_id' => false,
        'id' => false,
    ];
}
