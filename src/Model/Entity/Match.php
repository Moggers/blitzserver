<?php
namespace App\Model\Entity;

use Cake\ORM\Entity;

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

	protected function _getFalseName() {
		return str_replace( '_', ' ', $this->name );
	}

	protected function _getThrones() {
		return '' . $this->tone . '/' . $this->ttwo . '/' . $this->tthree . '(' . $this->points . ')';
	}

	protected function _getStatusString() {
		$str = match::statuses( $this->status );
		if( $str == '' ) {
			return 'N/A';
		}
		return $str;
	}

	protected function _getSafePort() {
		if( $this->status == 0 || $this->status == 2 ) {
			return 'N/A';
		} else {
			return $this->port;
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
			self::STATUS_CRIT_FAILURE => __('Nagot gick fel', true)];
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


	public static function nations( $value = null )
	{
		$options = [
			self::NATION_EARLY_ARCO => __('Arcoscephale, Golden Era'),
			self::NATION_EARLY_ERMO => __('Ermor, New Faith'),
			self::NATION_EARLY_ULM => __('Ulm, Enigma of Steel'),
			self::NATION_EARLY_MARV => __('Marverni, Time of Druids'),
			self::NATION_EARLY_SAUR => __('Sauromatia, Amazon Queens'),
			self::NATION_EARLY_TIEN => __('T\'ien Chi, Spring and Autumn'),
			self::NATION_EARLY_MACH => __('Machaka, Lion Kings'),
			self::NATION_EARLY_MICT => __('Mictlan, Reign of Blood'),
			self::NATION_EARLY_ABYS => __('Abysia, Children of Flame'),
			self::NATION_EARLY_CAEL => __('Caelum, Eagle Kings'),
			self::NATION_EARLY_CTIS => __('C\'tis, Lizard Kings'),
			self::NATION_EARLY_PANG => __('Pangaea, Age of Revelry'),
			self::NATION_EARLY_AGAR => __('Agartha, Pale Ones'),
			self::NATION_EARLY_TIRN => __('Tir na n\Og, Land of ELVES'),
			self::NATION_EARLY_FOMO => __('Fomoria, The Cursed Ones'),
			self::NATION_EARLY_VANH => __('Vanheim, Age of ELVES'),
			self::NATION_EARLY_HELH => __('Helheim, Dusk and ELVES'),
			self::NATION_EARLY_NIEF => __('Niefelheim, Sons of Winter'),
			self::NATION_EARLY_KAIL => __('Kailasa, Rise of the Ape Kings'),
			self::NATION_EARLY_LANK => __('Lanka, Land of Demons'),
			self::NATION_EARLY_YOMI => __('Yomi, Oni Kings'),
			self::NATION_EARLY_HINN => __('Hinnom, Sons of the Fallen'),
			self::NATION_EARLY_UR => __('Ur, The First City'),
			self::NATION_EARLY_BERY => __('Berytos, The Phoenix Empire'),
			self::NATION_EARLY_XIBA => __('Xibalba, Vigil of the Sun'),
			self::NATION_EARLY_ATLA => __('Atlantis, Emergence of the Deep Ones'),
			self::NATION_EARLY_RLYE => __('R\'lyeh, Time of Aboleths'),
			self::NATION_EARLY_PELA => __('Pelagia, Pearl Kings'),
			self::NATION_EARLY_OCEA => __('Oceania, Coming of the Capricorns'),
			self::NATION_EARLY_THER => __('Therodos, Telkhine Spectre')];
		return Match::enum( $value, $options );
	}
	public static function getNations( $value = null )
	{
		$ii = 0;
		$nationset = array();
		for( $ii = 0; $ii < 32; $ii++ ) {
			if( ($value >> $ii ) & 1 == 1 )
			{
				array_push( $nationset, match::nations( $ii) );
			}
		}
		return $nationset;
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

	const AGE_EARLY = 1;
	const AGE_MIDDLE = 2;
	const AGE_LATE = 3;

	const NATION_EARLY_ARCO = 5;
	const NATION_EARLY_ERMO = 6;
	const NATION_EARLY_ULM = 7;
	const NATION_EARLY_MARV = 8;
	const NATION_EARLY_SAUR = 9;
	const NATION_EARLY_TIEN = 10;
	const NATION_EARLY_MACH = 11;
	const NATION_EARLY_MICT = 12;
	const NATION_EARLY_ABYS = 13;
	const NATION_EARLY_CAEL = 14;
	const NATION_EARLY_CTIS = 15;
	const NATION_EARLY_PANG = 16;
	const NATION_EARLY_AGAR = 17;
	const NATION_EARLY_TIRN = 18;
	const NATION_EARLY_FOMO = 19;
	const NATION_EARLY_VANH = 20;
	const NATION_EARLY_HELH = 21;
	const NATION_EARLY_NIEF = 22;
	const NATION_EARLY_KAIL = 23;
	const NATION_EARLY_LANK = 24;
	const NATION_EARLY_YOMI = 25;
	const NATION_EARLY_HINN = 26;
	const NATION_EARLY_UR = 27;
	const NATION_EARLY_BERY = 28;
	const NATION_EARLY_XIBA = 29;
	const NATION_EARLY_ATLA = 30;
	const NATION_EARLY_RLYE = 31;
	const NATION_EARLY_PELA = 32;
	const NATION_EARLY_OCEA = 33;
	const NATION_EARLY_THER = 34;
}
