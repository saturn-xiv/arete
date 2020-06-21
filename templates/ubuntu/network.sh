# using ip instead of ifconfig

auto lo
iface lo inet loopback

{% match ether -%}
{% when Some with (item) -%}
{% let (name, eth) = item -%}
auto {{ name }}
allow-hotplug {{ name }}
{% match eth -%}
{% when Ether::Dhcp -%}
iface {{ name }} inet dhcp
{% when Ether::Static with {address, netmask, gateway, dns1, dns2} -%}
iface {{ name }} inet static
  address {{ address }}
  netmask {{ netmask }}
  gateway {{ gateway }}
  dns-nameservers {{ dns1 }} {{ dns2 }}
{% endmatch -%}
{% when None -%}
{% endmatch -%}


{% match wifi -%}
{% when Some with (item) -%}
{% let (name, _) = item -%}
auto {{ name }}
allow-hotplug {{ name }}
iface {{ name }} inet dhcp
  wpa-conf /etc/wpa_supplicant/wpa_supplicant.conf
{% when None -%}
{% endmatch -%}


{% match wifi -%}
{% when Some with (item) -%}
{% let (_, wifi) = item -%}
ctrl_interface=DIR=/var/run/wpa_supplicant
update_config=1
network={
{% match wifi -%}
{% when Wifi::Open with {ssid} %}
  ssid="{{ ssid }}"
{% when Wifi::Psk with {ssid, password} %}
  ssid="{{ ssid }}"  
  psk="{{ password }}"
{% when Wifi::Eap with {ssid, identity, password} %}
  ssid="{{ ssid }}"  
  password="{{ password }}"
  identity="{{ identity }}"
{% endmatch -%}
}
{% when None -%}
{% endmatch -%}
