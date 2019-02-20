#include <string>
#include <ctype.h>

class PhoneDir
{
  public:
    static std::string phone(const std::string &orgdr, std::string num)
    {
        std::size_t begin_number = orgdr.find(num);
        std::size_t end_number = begin_number + num.length();

        if (begin_number == std::string::npos)
        {
            return "Error => Not found: " + num;
        }

        std::size_t second_match = orgdr.find(num, begin_number + 1);

        if (second_match != std::string::npos)
        {
            return "Error => Too many people: " + num;
        }

        std::size_t current_position = begin_number;
        std::size_t begin_line = 0;
        std::size_t end_line = 0;

        while (current_position > 0 && orgdr[current_position] != '\n')
        {
            current_position -= 1;
        }

        begin_line = current_position;
        current_position = begin_number;

        while (current_position < orgdr.length() && orgdr[current_position] != '\n')
        {
            current_position += 1;
        }

        end_line = current_position;

        std::string number = orgdr.substr(begin_number, num.length());
        std::string name;
        std::string address;

        bool name_started = false;
        bool address_started = false;
        bool looking_for_new_word = false;

        for (size_t i = begin_line; i < end_line; i++)
        {
            if (name_started && orgdr[i] != '>')
            {
                name.push_back(orgdr[i]);
            }

            if (orgdr[i] == '<' && !name_started)
            {
                name_started = true;
            }
            else if (orgdr[i] == '>' && name_started)
            {
                name_started = false;
            }

            if (!name_started && (i < begin_number || i > end_number))
            {
                address_started = true;
            }
            else
            {
                address_started = false;
            }

            if (address_started)
            {
                if (looking_for_new_word && isalnum(orgdr[i]))
                {
                    address.push_back(' ');
                    address.push_back(orgdr[i]);
                    looking_for_new_word = false;
                }
                else if (!looking_for_new_word && (isalnum(orgdr[i]) || orgdr[i] == '.' || orgdr[i] == '-'))
                {
                    address.push_back(orgdr[i]);
                }
                else if (address.length() > 0 && !isalnum(orgdr[i]))
                {
                    looking_for_new_word = true;
                }
            }
        }

        if (!isalnum(address.back()))
        {
            address.pop_back();
        }

        return "Phone => " + number + ", Name => " + name + ", Address => " + address;
    }
};