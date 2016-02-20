<?php
namespace App\Model\Entity;

use Cake\ORM\Entity;
use Cake\Auth\DefaultPasswordHasher;
use Cake\I18n\Time;

/**
 * Match Entity.
 *
 * @property int $id
 * @property int $map_id
 * @property \App\Model\Entity\Map $map
 * @property int $age
 */
class Match extends Entity
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
        'id' => false,
    ];

	public static function enum( $value, $options, $default = '' ) {
		if( $value !== null ) {
			if( array_key_Exists( $value, $options )) {
				return $options[$value];
			}
		return $default;
		}
		return $options;
	}

	protected function _getNextTurn()
	{
		if( $this->hostinterval != 0 ) {
			Time::setToStringFormat('yyyy-MM-dd HH:mm:ss');	
			$clone = clone $this->lastturn;
			return $clone->addMinutes( $this->hostinterval );
		} else if( $this->hour != 0 ) {
			return false;
		}
		return false;
	}

	protected function _setPassword( $password )
	{
		if( strlen( $password ) > 0 ) {
			return (new DefaultPasswordHasher)->hash($password);
		}
	}

	protected function _getDayString()
	{
		switch( $this->day ) {
			case 0:
				return 'Sunday';
			case 1:
				return 'Monday';
			case 2:
				return 'Tuesday';
			case 3:
				return 'Wednesday';
			case 4:
				return 'Thursday';
			case 5:
				return 'Friday';
			case 6:
				return 'Saturday';
		}
	}

	public function checkPassword( $password )
	{
		if( (new DefaultPasswordHasher)->check($password, $this->password))
			return true;
		return false;
	}

	protected function _getFalseName() {
		return str_replace( '_', ' ', $this->name );
	}

	protected function _getThrones() {
		return '' . $this->tone . '/' . $this->ttwo . '/' . $this->tthree . '(' . $this->points . ')';
	}

	protected function _getStatusString() {
		$str = match::statuses( $this->status );
		if( $str == '' ) {
			return 'Unknown Status: '.$this->status;
		}
		return $str;
	}

	protected function _getPlayerCount() {
		return count($this->nations );
	}

	protected function _getAddress() {
		if( $this->status == 0 || $this->status == 2 ) {
			return 'N/A';
		} else {
			return SERVER_IP.":".$this->port;
		}
	}

	public static function statuses( $value = null )
	{
		$options = [
			self::STATUS_DELETED => __('Marked for Deletion', true ),
			self::STATUS_NEW => __('Pending', true ),
			self::STATUS_LOBBY => __('In Lobby', true ),
			self::STATUS_STARTED => __('Starting', true ),
			self::STATUS_RUNNING => __('Running', true ),
			self::STATUS_COUNTDOWN_5 => __('Starting in 5 seconds', true ),
			self::STATUS_COUNTDOWN_10 => __('Starting in 10 seconds', true ),
			self::STATUS_COUNTDOWN_15 => __('Starting in 15 seconds', true ),
			self::STATUS_CRIT_FAILURE => __('Nagot gick fel', true),
			self::STATUS_NO_PORTS => __('Ran out of ports', true)];
		return Match::enum( $value, $options );
	}

	public static function ages( $value = null )
	{
		$options = [
			self::AGE_EARLY => __('Early', true ),
			self::AGE_MIDDLE => __('Middle', true ),
			self::AGE_LATE => __('Late', true ) ];

		return Match::enum( $value, $options );
	}


	const STATUS_DELETED = -1;
	const STATUS_NEW = 0;
	const STATUS_LOBBY = 1;
	const STATUS_STARTED = 2;
	const STATUS_RUNNING = 3;
	const STATUS_COUNTDOWN_5 = 11;
	const STATUS_COUNTDOWN_10 = 12;
	const STATUS_COUNTDOWN_15 = 13;
	const STATUS_CRIT_FAILURE = 99;
	const STATUS_NO_PORTS = 101;

	const AGE_EARLY = 1;
	const AGE_MIDDLE = 2;
	const AGE_LATE = 3;
}
