#include "MedicalLib/Organ.h"

Organ::Organ(int id, const std::string& type) : organId(id), organType(type) {}

int Organ::getId() const {
    return organId;
}

const std::string& Organ::getType() const {
    return organType;
}
